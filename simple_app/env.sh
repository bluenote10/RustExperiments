# https://unix.stackexchange.com/a/501488/63722
# https://stackoverflow.com/questions/9901210/bash-source0-equivalent-in-zsh
source_file_path=${BASH_SOURCE[0]:-${(%):-%x}}
export PROJECT_ROOT=$(readlink -f $(dirname $source_file_path))
echo "\$PROJECT_ROOT=${PROJECT_ROOT}"

export VENV_DIR="${PROJECT_ROOT}/venv"
echo "\$VENV_DIR=${VENV_DIR}"

if [ -e "$VENV_DIR/bin/activate" ]
then
  . "$VENV_DIR/bin/activate"
  echo "Venv: activated"
else
  echo "Venv doesn't seem to exist. Create via 'venv_create'"
fi

# Note that it is better to modify $PATH _after_ activating the venv, because
# activating a venv can lead to a reset of $PATH if there is a previously
# activated venv (which implicitly gets deactived, resetting $PATH to the
# state it had when the old venv got activeted).
export PATH="${PROJECT_ROOT}/scripts:${PATH}"
