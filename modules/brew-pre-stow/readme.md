# brew-pre-stow

For brew to be fully installed, stow is needed to link files, to make brew
available on the PATH etc. But stow is installed through brew...

So this module installs brew then stow immediately. This module also most not
contain anything to stow.
