#!/bin/bash

# usage: ./ocd.sh {start|stop|restart|status|log}
# optional env: INSTANCE=0 DEVICE=1 CABLE=1 GDB_PORT=... TELNET_PORT=...

QUARTUS_ROOT=${QUARTUS_ROOT:-$HOME/altera_lite/25.1std}
export CABLE=${CABLE:-1} DEVICE=${DEVICE:-1} INSTANCE=${INSTANCE:-0}
export GDB_PORT=${GDB_PORT:-$((3333 + INSTANCE))}
export TELNET_PORT=${TELNET_PORT:-$((4444 + INSTANCE))}

DIR="$(cd "$(dirname "$0")" && pwd)"
export DBG="$DIR/.debug"
export CFG="$DBG/niosv$INSTANCE.cfg"
export LOG="$DBG/openocd$INSTANCE.log"
export PIDFILE="$DBG/openocd$INSTANCE.pid"

mkdir -p "$DBG"
[ -f "$DBG/.gitignore" ] || echo '*' > "$DBG/.gitignore"

is_running() { [ -f "$PIDFILE" ] && kill -0 "$(cat "$PIDFILE")" 2>/dev/null; }

start() {
  if is_running; then
    echo "already running (pid $(cat "$PIDFILE"))"; return 0
  fi
  : > "$LOG"
  setsid "$QUARTUS_ROOT/niosv/bin/niosv-shell" >>"$LOG" 2>&1 <<'EOF' &
cd "$DBG"
[ -f "$CFG" ] || openocd-cfg-gen --cable=$CABLE --device=$DEVICE --instance=$INSTANCE "$CFG" || exit 1
echo $$ > "$PIDFILE"
exec openocd -c "tcl_port disabled" -c "gdb_port $GDB_PORT" -c "telnet_port $TELNET_PORT" -f "$CFG" </dev/null >>"$LOG" 2>&1
EOF
  for _ in $(seq 1 20); do
    sleep 0.5
    if grep -q "Listening on port $GDB_PORT" "$LOG"; then
      echo "up: gdb on :$GDB_PORT (pid $(cat "$PIDFILE"))"; return 0
    fi
  done
  echo "did not come up, last log lines:"; tail -n 15 "$LOG"; return 1
}

stop() {
  if is_running; then kill "$(cat "$PIDFILE")"; echo "stopped"; else echo "not running"; fi
  rm -f "$PIDFILE"
}

case "$1" in
  start)   start ;;
  stop)    stop ;;
  restart) stop; sleep 1; start ;;
  status)  is_running && echo "running (pid $(cat "$PIDFILE")), gdb on :$GDB_PORT" || echo "not running" ;;
  log)     tail -f "$LOG" ;;
  clean)   stop; rm -f "$DBG"/niosv*.cfg "$DBG"/openocd*.log ;;
  *)       echo "usage: $0 {start|stop|restart|status|log|clean}"; exit 1 ;;
esac