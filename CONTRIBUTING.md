# Contributing
This document contains information that may be useful for contributors to the Prebuilt-MPR GitHub repository.

## The Prebuilt-MPR update flow
There's a couple steps that happen between when a package update is pushed on the MPR to when that package gets into the Prebuilt-MPR APT repository. That process is described below.

### 1. The update checker
The first thing that happens to get a package into the Prebuilt-MPR is for the system to know that the package needs to be updated in the Prebuilt-MPR in the first place. This is run as a 
