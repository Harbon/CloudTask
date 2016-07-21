#!/bin/sh

if [ -e $1 ];then
  zbarimg --raw $1 > qrcode.txt;
  result=`cat qrcode.txt`;
  if [ $result -eq ""];then
    echo "NOTQRCODE" > qrcode.txt;
  fi
  rm -f $1;
else
  echo "NOTPICTURE" > qrcode.txt;
fi
