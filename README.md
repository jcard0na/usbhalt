# Raspberry Pi USB Halt

The Raspberry Pi has no shut down button.  Shutting down the system by cycling power [might corrupt the SD card and leave the system unbootable](https://www.raspberrypi.org/forums/viewtopic.php?t=219032).


In order to avoid SD card corruption, on needs to properly shut down the
system.  That is difficult on embedded systems without a console.  People have
[designed reset buttons that can trigger a shutdown via GPIO](
https://scruss.com/blog/2017/10/21/combined-restart-shutdown-button-for-raspberry-pi/
).  That works well if GPIO is accessible, but sometimes it is not.   

In that case, you can use this hack:  use the USB port to trigger a USB error
that triggers a shutdown.  It cannot be uglier, but it works very well.

You will need a USB-A connector connected to a button that shorts Vbus (5V,
typically the red cable) with Ground (GNC, typically the black cable).

Insert it on any USB port, launch this damon and press the button to halt.
