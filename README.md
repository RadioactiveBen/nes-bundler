# NES Bundler

**Transform your NES-game into a single executable targeting your favourite OS!**

Did you make a NES-game but none of your friends own a Nintendo? Don't worry.  
Add your ROM and configure NES Bundler to build for Mac, Windows and Linux.  
What you get is an executable with
* Simple UI for settings (Show and hide with ESC).
* Re-mappable Keyboard and Gamepad input (you bundle your default mappings).
* Save/Restore state (F1 = Save, F2 = Load).
* Netplay! (Optional feature, can be disabled if not wanted).

<p align="center">
  <img src="https://github.com/tedsteen/nes-bundler/blob/master/screenshot.png?raw=true" alt="Super Mario!"/>
</p>

## Try it out

Before you make a proper bundle with your own icons and installer graphics you can try out NES Bundler by downloading the [binary of your choice](https://github.com/tedsteen/nes-bundler/releases/).  
Running that will start a demo bundle, but if you place your own [config.yaml and rom.nes](config/) in the same directory as the executable it will use that.

## Proper bundling

To create a bundle you first need to configure it with your ROM and a bundle configuration and then let Github actions build it.  
To do this you need to create a PR with your own [configuration](config/README.md).  
Why you ask? Because in order to build for all platforms and digitally sign the executables you need a lot of setup. It is possible to do it yourself and you can read all about it in the configuration readme.  

Perhaps in the future there will be a more convenient way to build a bundle.

## Limitations

* It's built on [rusticnes-core](https://github.com/zeta0134/rusticnes-core) so it's limited to what that can emulate (f.ex no PAL).
* Save/load state and thus netplay is currently working for mappers nrom, mmc1, mmc3, uxrom, axrom, bnrom, cnrom, gxrom and ines31.  
  If you want to contribute, please implement save/load for a mapper [over here](https://github.com/tedsteen/rusticnes-core-for-nes-bundler/blob/master/src/mmc/mapper.rs#L43-L45).