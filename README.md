# tomat

[![Build Status](https://travis-ci.com/aydoganersoz/tomat.svg?branch=master)](https://travis-ci.com/aydoganersoz/tomat)

_tomat_ is yet another console based [pomodoro timer](https://en.wikipedia.org/wiki/Pomodoro_Technique). It has very limited features and has been created for fun during my Rust learning adventure.

# Features

- Pomodoro duration is 25, short break duration is 5 and long break duration is 15 minutes by default
- Session durations can be modified individually or together
- First two pomodoro sessions are followed by short breaks and the third one by a long break, then this pattern repeats (i.e.: pomodoro -> short break -> pomodoro -> short break -> pomodoro -> long break)
- A yes/no confirmation dialogue to continue shows up at the end of each session to give flexibility to the user
- A beep sound is played at the end of each session to inform the user
- A completed session is recorded in the daily statistics
- Daily statistics can be shown on the terminal
- Daily statistics can be exported into a JSON file
- Daily statistics can be reset

# Installation

## On Linux & OSX

Download the installation script:

```bash
curl -L -o tomat_install.sh https://raw.githubusercontent.com/aydoganersoz/tomat/master/install.sh
```

Give execution permission to the script:

```bash
chmod +x ./tomat_install.sh
```

Install _tomat_

```bash
./tomat_install.sh
```

# Usage

## Start the timer

### With default durations

```
tomat start
```

### With user-defined durations

`start` subcommand can take three optional parameters to modify session durations. For example, following command can be used to modify the pomodoro duration to 30 minutes, the short break duration to 10 minutes and the long break duration to 20 minutes:

```
tomat start --pomodoro=30 --short-break=10 --long-break=20
```

## View daily statistics

```
tomat stat show
```

## Export daily statistics

```
tomat stat export
```

Exported files can be found under `$HOME\.tomat\out`. Unix timestamp prefix is used in filenames to generate unique files that can be ordered by creation date.

## Reset statistics

```
tomat stat reset
```

## Quit the timer

There are two methods to quit the timer. Pressing `CTRL^C` at anytime or answering `no` to the confirmation dialogue that shows up at the end of each session.

# Credits

- [Elif Koksal](https://github.com/elifkoksal) for her valuable feedback
- SoundBible.com for A-Tone-His_Self-1266414414.wav