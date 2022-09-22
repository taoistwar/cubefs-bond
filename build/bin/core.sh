#!/usr/bin/env bash
BASE="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )"
cd $BASE

################################
# params
################################
#parse comand name, parameter
OPERATE=$1
shift
params=$@

################################
# constants
################################

#dir
APP_NAME=cubefs-bond
APP_CONF="$BASE/conf"
APP_LOGS="$BASE/logs"
APP_LIB="$BASE/lib"
APP_PID="$BASE/pid"

#file
APP_OUT="${APP_LOGS}/${APP_NAME}.out"

#port
APP_PORT=`cat ${APP_CONF}/cubefs-bond.toml|grep 'port ='|cut -d '=' -f 2|cut -d ' ' -f 2`


################################
# functions
################################
# Shell Colors
GREEN=$'\e[0;32m'
LGREEN=$'\e[1;32m'
RED=$'\e[0;31m'
LRED=$'\e[1;31m'
BLUE=$'\e[0;34m'
LBLUE=$'\e[1;34m'
RESET=$'\e[m'
function error() {
    debug error $@
}
function debug() {
    if [[ "$1" == "warn" ]]; then
        shift
        echo -e "     ${LBLUE}$1${RESET}"
    elif [[ "$1" == "info" ]]; then
        shift
        echo -e "     ${BLUE}$1${RESET}"
    elif [[ "$1" == "error" ]]; then
        shift
        echo -e "     ${RED}ERROR:${LRED} $@${RESET}"
        exit 1
    else
        echo -e $@
    fi
}
function cmd() {
    echo -e  "CMD: $OPERATE \t $params"
}

function env() {

        cat << EOF
ENV:
        APP_CONF: ${APP_CONF}
        APP_LOGS: ${APP_LOGS}
        APP_OUT : ${APP_OUT}
EOF
}

function init() {
        mkdir -p ${APP_CONF} ${APP_LOGS}
}

function check() {
    PID=`ps aux|grep 'cubefs-bond -d'|grep -v grep|awk '{print $2}'`
    if [ -n "$PID" ]; then
        debug error "The ${APP_NAME} already started! PID: $PID, port:${APP_PORT}"
        ps aux|grep 'cubefs-bond -d'|grep -v grep
        exit 1
    fi
}

function start() {
    check
    cd $BASE
    nohup $BASE/bin/cubefs-bond ${params} > ${APP_OUT} 2>&1 &
    PID=$!
    debug info "${APP_NAME}(pid ${PID}, port:${APP_PORT}) is started."
    ps aux|grep 'cubefs-bond -d'|grep -v grep
}

function status() {
    PID=`ps aux|grep 'cubefs-bond -d'|grep -v grep|awk '{print $2}'`
    if [ -n "$PID" ]; then
        debug info "${APP_NAME}(pid ${PID}, port:${APP_PORT}) is running..."
        ps aux|grep 'cubefs-bond -d'|grep -v grep
    else
        debug info "${APP_NAME} is not running."
    fi
}

function stop() {
    PID=`ps aux|grep 'cubefs-bond -d'|grep -v grep|awk '{print $2}'`
    if [ -n "$PID" ]; then
        kill ${PID}
        debug info "${APP_NAME}(pid ${PID}) is stopped"
        exit 0
    else
        debug error "${APP_NAME} is not running."
    fi
}

function run() {
  init
  cmd
  env

  echo "RES:"

  case $OPERATE in
    start)
      start
      ;;
    stop)
      stop
      ;;
    status)
      status
      ;;
  esac
}

################################
# run
################################
run