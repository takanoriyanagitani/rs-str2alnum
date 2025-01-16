#!/bin/sh

strings(){
	echo -- printing $1 --
	echo 'hello, world!'
	echo 'hello_world'
	echo -- done --
	echo
}

strings original
strings filtered | ./rs-str2alnum

export ENV_ALLOW_UNDER_SCORE=true
strings allow_us | ./rs-str2alnum
