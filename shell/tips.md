SHELL TIPS (as H1)
==================
start_at: `2014-02-13`

## indent with 4 space

    if [ $count -eq 1 ]; then
        echo "good"
    fi

## check cmd exists

    if type -p curl >/dev/null 2>&1; then
    fi

## dirname

    LEIN_DIR="$(dirname "$BIN_DIR")"

## echo -n
no new line
    
    echo -n "hello,world"

## exec sh -c 

    exec sh -c "exec $TRAMPOLINE"