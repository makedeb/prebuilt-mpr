# Contributing
This document contains information that may be useful for maintainers of the Prebuilt-MPR GitHub repository.

## The Prebuilt-MPR update flow
There's a couple steps that happen between when a package update is pushed on the MPR to when that package gets into the Prebuilt-MPR APT repository.   
<sub>Some technical details have been left out, but the main points of interest are still listed.</sub>

### 1. Update checks
All packages run through a [GitHub Actions job](https://github.com/makedeb/prebuilt-mpr/actions/workflows/check-pkg.yml) that compares the version of a package on the MPR to that in the Prebuilt-MPR. If the version on the MPR is higher than that in the Prebuilt-MPR, the steps below get ran.

### 2. The Pull Request
Next, a Pull Request gets created, containing the changes introduced in the new package version. After [the package is automatically built](https://github.com/makedeb/prebuilt-mpr/actions/workflows/update-pkg.yml) and reviewed by a Prebuilt-MPR team member, the package is ready to be updated in the Prebuilt-MPR APT repository.

### 3. The package update
The package has now been built and reviewed by a Prebuilt-MPR team member, and is ready to be updated in the Prebuilt-MPR APT repository. The updated package is [uploaded to the APT repository](https://github.com/makedeb/prebuilt-mpr/actions/workflows/publish-pkg.yml), and is ready to be downloaded for use by end users.
