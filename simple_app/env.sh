# https://unix.stackexchange.com/a/501488/63722
# https://stackoverflow.com/questions/9901210/bash-source0-equivalent-in-zsh
source_file_path=${BASH_SOURCE[0]:-${(%):-%x}}
export PROJECT_ROOT=$(readlink -f $(dirname $source_file_path))
echo "\$PROJECT_ROOT=$PROJECT_ROOT"

export VENV_DIR=${PROJECT_ROOT}/venv
echo "\$VENV_DIR=$VENV_DIR"

export PATH=${PROJECT_ROOT}/scripts:${PATH}

if [ -e "$VENV_DIR/bin/activate" ]
then
  . "$VENV_DIR/bin/activate"
  echo "Venv: activated"
else
  echo "Venv doesn't seem to exist. Create via 'venv_create'"
fi
