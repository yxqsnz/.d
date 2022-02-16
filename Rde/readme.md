# Description
RDE - Rem's Desktop Environment

RDE is currently in version 0.1 beta

fully written in Rust, RDE is a Desktop Environment meant to be used through a terminal/tty. its Ncurses Based.

# How To Use

first of all compile it lol.

then run it. press ctrl + d, this is the App Launcher, at the moment RDE has only 2 applications, the text editor, and the filemanager.

launch the filemanager through the app launcher, by typing files `<path>`

launch the editor through the app launcher by typing editor `<filename>`

# Preview
 
 ![App Launcher](https://i.imgur.com/CRLcMCt.png)
 ![TextEditor](https://i.imgur.com/ORVPxKA.png)


# installation Steps

RDE depends on NCurses, and some Linux Systems doesnt have NCurses installed by default


ubuntu/debian
```zsh
sudo apt-get install libncurses5-dev libncursesw5-dev
```
Fedora
```zsh
sudo dnf install ncurses-devel
```
CentOS/RHEL
```zsh
sudo yum install ncurses-devel
```
ArchLinux
```zsh
sudo pacman -S ncurses
```
