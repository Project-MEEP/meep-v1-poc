#!/usr/bin/env bash
# SEC-05 items 1 and 2: SQL only as static text through the storage wrapper (check 5: static SQL inside crates/meep-storage/src, with the one D11 key-statement exception); templates escape by default.
# Fails on: Box::leak, String::leak or Vec::leak anywhere; set_use_isolating(false) outside test code; rusqlite use outside crates/meep-storage; `|safe` in any template. Check 5 fails on: a format!, format_args!, concat!, write!, writeln! or .push_str( line in crates/meep-storage/src without the D11 marker; a .execute(, .execute_batch(, .prepare(, .prepare_cached(, .query_row( or .query_map( call whose first argument is neither a string literal nor an UPPER_SNAKE_CASE constant, without the marker.
# Passes while no Rust or templates exist.
set -u
fail=0
rs=$(git ls-files '*.rs')
if [ -n "$rs" ]; then
  hits=$(printf '%s\n' "$rs" | xargs -d '\n' grep -nE 'Box::leak|String::leak|Vec::leak' 2>/dev/null || true)
  [ -n "$hits" ] && { echo "FAIL: Box::leak, String::leak or Vec::leak (SEC-05 item 1):"; echo "$hits"; fail=1; }
  iso=""
  while IFS= read -r f; do
    # join lines so a call split across lines is still matched
    if tr '\n\r' '  ' < "$f" | grep -qE 'set_use_isolating[[:space:]]*\([[:space:]]*false'; then iso="$iso$f"$'\n'; fi
  done < <(printf '%s\n' "$rs" | grep -vE '/tests/|_test\.rs$')
  [ -n "$iso" ] && { echo "FAIL: Fluent isolation turned off outside tests (SEC-05 item 5):"; echo "$iso"; fail=1; }
  outside=$(printf '%s\n' "$rs" | grep -v '^crates/meep-storage/' || true)
  if [ -n "$outside" ]; then
    hits=$(printf '%s\n' "$outside" | xargs -d '\n' grep -nE 'use rusqlite|rusqlite::|\.(execute_batch|prepare|prepare_cached)\(' 2>/dev/null || true)
    [ -n "$hits" ] && { echo "FAIL: database access outside crates/meep-storage (SEC-05 item 1):"; echo "$hits"; fail=1; }
  fi
fi
tpl=$(git ls-files | grep -E '\.(html|htm|jinja|j2|askama)$|/templates/' | grep -v '^specs/\|^docs/' || true)
if [ -n "$tpl" ]; then
  hits=$(printf '%s\n' "$tpl" | xargs -d '\n' grep -nE '\|[[:space:]]*safe\b|escape[[:space:]]*=[[:space:]]*"?none' 2>/dev/null || true)
  [ -n "$hits" ] && { echo "FAIL: |safe or disabled escaping in a template (SEC-05 item 2):"; echo "$hits"; fail=1; }
fi
# Check 5 (SEC-05 item 1, owner decision D11-A): SQL in crates/meep-storage/src is static text.
# The one allowed exception is the SQLCipher key statement, whose line ends with the marker below.
marker='// SEC-05 exception (D11)'
marker_re='// SEC-05 exception \(D11\)'
src=$(git ls-files 'crates/meep-storage/src/*.rs')
if [ -n "$src" ]; then
  (
    export LC_ALL=C   # byte offsets below must match bytes
    bad=0
    # (a) no text-building macro or push_str unless the same line ends with the marker
    hits=$(printf '%s\n' "$src" | xargs -d '\n' grep -nHE 'format!|format_args!|concat!|write!|writeln!|\.push_str\(' 2>/dev/null | grep -vE "${marker_re}[[:space:]]*$" || true)
    [ -n "$hits" ] && { echo "FAIL: text built at run time in crates/meep-storage/src without the D11 marker (SEC-05 item 1):"; echo "$hits"; bad=1; }
    # (b) the first argument of every SQL call is a string literal or an UPPER_SNAKE_CASE constant
    sql_hits=""
    while IFS= read -r f; do
      joined=$(tr '\n\r' '  ' < "$f")   # one line, as check 2 does; offsets stay equal to file bytes
      while IFS=: read -r off _; do
        [ -n "$off" ] || continue
        call=${joined:$off}
        call=${call%%;*}   # up to the next semicolon
        case $call in *"$marker"*) continue ;; esac
        arg=${call#*\(}
        arg=${arg#"${arg%%[![:space:]]*}"}
        if [[ $arg =~ ^(\"|r\"|r#\") ]] || [[ $arg =~ ^[A-Z][A-Z0-9_]*[[:space:]]*[,\)] ]]; then continue; fi
        line=$(( $(head -c "$off" "$f" | tr -cd '\n' | wc -c) + 1 ))
        sql_hits="$sql_hits$f:$line: ${call:0:70}"$'\n'
      done < <(printf '%s' "$joined" | grep -obE '\.(execute|execute_batch|prepare|prepare_cached|query_row|query_map)\(')
    done <<< "$src"
    [ -n "$sql_hits" ] && { echo "FAIL: SQL that is not a string literal or an UPPER_SNAKE_CASE constant in crates/meep-storage/src (SEC-05 item 1):"; printf '%s' "$sql_hits"; bad=1; }
    exit "$bad"
  ) || fail=1
fi
[ "$fail" -eq 0 ] && echo "check-sec05: clean"
exit "$fail"
