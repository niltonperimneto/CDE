dtscreenuser cmddtscreenthe
Common Desktop Environment animated screen saversdtscreen-displaydsp-delayusecs-batchcountnum-saturationvalue-nicenicelevel-modemodeDESCRIPTIONThe dtscreen client supports the following key tasks.- Draws an animated image on a provided window id(s).- Does not assume the size of the provided window(s).- Provides a set of selectable animated images.- Animation parameters such as nice, speed, number and saturation may
be specified.The dtscreen client provides the default screen saver animations for
the desktop. When launched, it will query the desktop using the API for the
set for window ids on which to draw. These window ids could be the cover windows
created by the session manager for session lock, or a window embedded in the
style manager's session lock customization dialog. Once obtained, the dtscreen
client will proceed to draw in these windows using the specified options.
If dtscreen cannot obtain the set of window ids, it will create its own. Normally,
the dtscreen client will be invoked directly by the session manager, or by
the style manager.Note that the Session Manager, dtsession, is responsible for locking
the session and prompting for a password to unlock.The session manager may launch the dtscreen client to provide screen
saver animations during session lock. Refer to the session manager specification
for resources controlling the launching of screen savers by the session manager.The style manager may launch the dtscreen client to provide a preview
of a screen saver animation during customization. Refer to the style manager
specification for information on how to integrate a screen saver client such
as dtscreen into the desktop.The dtsession client provides a selectable set of animations. These
are:hopHopalong iterated fractals.lifeConway's game of life.qixSpinning lines.imageRandom bouncing image.swarmSwarm of bees.rotorRotorpyroFireworks.flameCosmic Flame Fractals.wormWiggly Worms.randomRandom mode.The dtscreen client animations are based on xlock(1).OPTIONS-displaydspThe display on which animations will be drawn.-delayusecsThe delay option sets the speed at which a mode will operate. It simply
sets the number of microseconds to delay between batches of "hopalong pixels",
"qix lines", "life generations", "image blits", and "swarm motions".-batchcountnumThe batchcount option sets the number of things to do per batch to num.
In qix mode this refers to the number of lines rendered in the same color.
In hop mode this refers to the number of pixels rendered in the same color.
In image mode this refers to the number of X logos on the screen at once.
in swarm mode this refers to the number of bees. In life mode it means nothing.-saturationvalueThe saturation option sets the saturation of the color ramp used to
value. 0 is grayscale, 1 is very rich color and 0.4 is a nice pastel.-nicenicelevelThe nice option sets the system nicelevel of the dtscreen process to
nicelevel.-modemodeThe mode option specifies which animation should be displayed. Values
are:hopHopalong iterated fractals.lifeConway's game of life.qixSpinning lines.imageRandom bouncing image.swarmSwarm of bees.rotorRotorpyroFireworks.flameCosmic Flame Fractals.wormWiggly Worms.randomRandom mode.RETURN VALUEExit values are:0Successful completion.>0Error condition occurred.EXAMPLESdtscreen -mode swarmDisplay the swarm animation.RESOURCESNameClassClassTypeDefaultmodeModeStringswarmniceNiceInt16delayDelayInt1000batchcountBatchcountInt100saturationSaturationFloat1.0hop.delayDelayInt0hop.batchcountBatchcountInt1000hop.saturationSaturationFloat1image.delayDelayInt2000000image.batchcountBatchcountInt8image.saturationSaturationFloat0.2qix.delayDelayInt30000qix.batchcountBatchcountInt64qix.saturationSaturationFloat1life.delayDelayInt1000000life.batchcountBatchcountInt1life.saturationSaturationFloat1swarm.delayDelayInt10000swarm.batchcountBatchcountInt100swarm.saturationSaturationFloat1rotor.delayDelayInt10000rotor.batchcountBatchcountInt4rotor.saturationSaturationFloat0.4pyro.delayDelayInt15000pyro.batchcountBatchcountInt40pyro.saturationSaturationFloat1.0flame.delayDelayInt10000flame.batchcountBatchcountInt20flame.saturationSaturationFloat1.0worm.delayDelayInt10000worm.batchcountBatchcountInt20worm.saturationSaturationFloat1.0delayThe delay option sets the speed at which a mode will operate. It simply
sets the number of microseconds to delay between batches of "hopalong pixels",
"qix lines", "life generations", "image blits", and "swarm motions".batchcountThe batchcount option sets the number of things to do per batch to num.
In qix mode this refers to the number of lines rendered in the same color.
In hop mode this refers to the number of pixels rendered in the same color.
In image mode this refers to the number of X logos on the screen at once.
in swarm mode this refers to the number of bees. In life mode it means nothing.saturationThe saturation option sets the saturation of the color ramp used to
value. 0 is grayscale, 1 is very rich color and 0.4 is a nice pastel.niceThe nice option sets the system nicelevel of the dtscreen process to
nicelevel.modeThe mode option specifies which animation should be displayed. Values
are:hopHopalong iterated fractals.lifeConway's game of life.qixSpinning lines.imageRandom bouncing image.swarmSwarm of bees.rotorRotorpyroFireworks.flameCosmic Flame Fractals.wormWiggly Worms.randomRandom mode.SEEdtsession