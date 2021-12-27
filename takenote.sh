#!/usr/bin/env zsh

# Make commit.
gtakenote() {
  takenote2 $#
}

takenote2() {
  # Store location of call.
  workDir=`pwd`

  # Location to store journal notes. 
  # TODO: Migration from default directory (if any exists).
  DEFAULT_JOURNAL_DIR="~/.journal"
  journalDir="${TAKENOTE_JOURNAL_DIR:-$DEFAULT_JOURNAL_DIR}"

  # Create directory if it does not exist.
  if [[ ! -d $journalDir ]]
  then
    mkdir $journalDir
  fi

  # TODO: Use an alternative datastructure to optimize search.
  # Current: O(N) where N is the number of files.
  timestamp=`date "%s"`
  
  noteFile="$journalDir/$timestamp"

  echo "$notepad"

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

  note="$1"
  echo "$note"

  #echo "$note" > $notepad
  #cd $journal

  #git add .
  #git commit -m "'$note'"
  #git push

  #cd $work_dir

  #echo "Made a commit with note: $note"
}
