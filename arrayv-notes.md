notes taken while perusing the code of arrayV

## Notes on the ArrayManager Class
1. They use multiple different types of shuffles, figure out how they differ from eachother, and how I would implement them
2. They use different types of "distribution types" whatever that means 
3. 




# Notes on how they handled multithreading as a whole
    Relevant code:
    ``` java
public void runSort(int[] array, int selection) {
        if (arrayVisualizer.isActive())
            return;

        //TODO: This code is bugged! It causes the program to forget the sleep ratio specified by the user!
        if (delayOps.skipped()) {
            delayOps.setSleepRatio(1);
            delayOps.changeSkipped(false);
        }

        double storeVol = sounds.getVolume();
        sounds.toggleSound(true);
        arrayVisualizer.setSortingThread(new Thread("ComparisonSorting") {
            @Override
            public void run() {
                try {
                    SortInfo sortInfo = arrayVisualizer.getSorts()[selection];
                    int extra;

                    if (sortInfo.getQuestion() != null) {
                        try {
                            extra = sortInfo.validateAnswer(getCustomInput(sortInfo.getQuestion()));
                        } catch (Exception e) {
                            extra = sortInfo.getDefaultAnswer();
                        }
                    } else if (sortInfo.isBucketSort()) {
                        if (sortInfo.isRadixSort()) {
                            try {
                                extra = getCustomInput("Enter the base for this sort:");
                            } catch (Exception e) {
                                extra = 4;
                            }
                        } else if (sortInfo.getRunName().contains("Shatter")) {
                            try {
                                extra = getCustomInput("Enter the size for each partition:");
                            } catch (Exception e) {
                                extra = arrayVisualizer.getCurrentLength() / 16;
                            }
                        } else {
                            try {
                                extra = getCustomInput("How many buckets will this sort use?");
                            } catch (Exception e) {
                                extra = 16;
                            }
                        }
                        if (extra < 2) extra = 2;
                    } else {
                        extra = 0;
                    }

                    boolean goAhead;

                    if (sortInfo.getRunName().equals("Timesort")) {
                        Object[] options = { "Continue", "Cancel" };

                        int warning = JOptionPane.showOptionDialog(arrayVisualizer.getMainWindow(), "Time Sort will take at least " + getTimeSortEstimate(extra)
                                                                 + "to complete. Once it starts, you cannot skip this sort.", "Warning!", JOptionPane.OK_CANCEL_OPTION, JOptionPane.WARNING_MESSAGE,
                                                                   null, options, options[1]);

                        goAhead = warning == 0;
                    } else if (sortInfo.getUnreasonableLimit() > 0 && arrayVisualizer.getCurrentLength() > sortInfo.getUnreasonableLimit()) {

                        Object[] options = { "Let's see how bad " + sortInfo.getRunName() + " is!", "Cancel" };

                        int warning;
                        if (sortInfo.isBogoSort()) {
                            warning = JOptionPane.showOptionDialog(arrayVisualizer.getMainWindow(), "Even at a high speed, "
                                                                       + sortInfo.getRunName() + "ing " + arrayVisualizer.getCurrentLength()
                                                                       + " numbers will almost certainly not finish in a reasonable amount of time. "
                                                                       + "Are you sure you want to continue?", "Warning!", JOptionPane.OK_CANCEL_OPTION, JOptionPane.WARNING_MESSAGE,
                                                                   null, options, options[1]
                            );
                        } else {
                            warning = JOptionPane.showOptionDialog(arrayVisualizer.getMainWindow(), "Even at a high speed, "
                                                                       + sortInfo.getRunName() + "ing " + arrayVisualizer.getCurrentLength()
                                                                       + " numbers will not finish in a reasonable amount of time. "
                                                                       + "Are you sure you want to continue?", "Warning!", JOptionPane.OK_CANCEL_OPTION, JOptionPane.WARNING_MESSAGE,
                                                                   null, options, options[1]
                            );

                        }
                        goAhead = warning == 0;
                    } else {
                        goAhead = true;
                    }

                    if (goAhead) {
                        Sort sortInstance = sortInfo.getFreshInstance();

                        if (sortInfo.getRunName().equals("In-Place LSD Radix")) {
                            sounds.changeVolume(0.01); // Here to protect your ears :)
                        }

                        arrayManager.toggleMutableLength(false);
                        arrayManager.refreshArray(array, arrayVisualizer.getCurrentLength(), arrayVisualizer);

                        arrayVisualizer.setHeading(sortInfo.getRunName());
                        arrayVisualizer.setCategory(sortInfo.getCategory());

                        realTimer.enableRealTimer();
                        boolean antiq = arrayVisualizer.useAntiQSort();
                        boolean networks = arrayVisualizer.generateSortingNetworks();
                        if (antiq)
                            arrayVisualizer.initAntiQSort();
                        try {
                            sortInstance.runSort(array, arrayVisualizer.getCurrentLength(), extra);
                        } catch (StopSort ignored) {
                        } catch (OutOfMemoryError e) {
                            JErrorPane.invokeCustomErrorMessage(sortInfo.getRunName() + " ran out of memory: " + e.getMessage());
                            throw new RuntimeException(e);
                        }

                        if (antiq)
                            arrayVisualizer.finishAntiQSort(sortInfo.getClass().getSimpleName());
                        else if (networks) {
                            ArrayList<Integer> indicesList = arrayVisualizer.getReads().getNetworkIndices();
                            SortingNetworkGenerator.encodeNetworkAndDisplay(
                                sortInfo.getClass().getSimpleName(),
                                indicesList,
                                arrayVisualizer.getCurrentLength()
                            );
                        }
                    } else {
                        arrayManager.initializeArray(array);
                    }
                } catch (Exception e) {
                    JErrorPane.invokeErrorMessage(e);
                }
                arrayVisualizer.endSort();
                arrayManager.toggleMutableLength(true);
                sounds.changeVolume(storeVol);
                sounds.toggleSound(false);
                System.gc(); // Reduce RAM usage from any high-memory tasks (e.g. visualizing a sorting network)
            }
        });

        arrayVisualizer.runSortingThread();
    }
}
    
    ```
