#!/usr/bin/env zsh

takenote() {
  work_dir=$PWD

  journal="$HOME/personal/journal"

  year="$(date "+%Y")"
  week="$(date "+%U")"

  notepad="$journal/$year/$week.md"

  if [[ ! -f $notepad ]] 
  then
    touch "$notepad"
  fi

  if [[ -z $1 ]] 
  then
    cd $work_dir 
    echo "MESSAGE IS REQUIRED!"
    return 1
  fi

  $note="$1"

  echo "$note" > $notepad
  #cd $journal

  #git add .
  #git commit -m "'$note'"
  #git push

  #cd $work_dir

  #echo "Made a commit with note: $note"
}
