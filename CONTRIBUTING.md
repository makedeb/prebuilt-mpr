# Contributing
This document contains information that may be useful for maintainers of the Prebuilt-MPR GitHub repository.

## The Prebuilt-MPR update flow
There's a couple steps that happen between when a package update is pushed on the MPR to when that package gets into the Prebuilt-MPR APT repository.   
<sub>Some technical details have been left out, but the main points of interest are still listed.</sub>

### 1. Update checks
All packages run through a [GitHub Actions job](https://github.com/makedeb/prebuilt-mpr/actions/workflows/check-pkg.yml) that compares the version of a package on the MPR to that in the Prebuilt-MPR. If the version on the MPR is higher than that in the Prebuilt-MPR, the steps below get ran.

### 2. The pull request
Next, a pull request gets created, containing the changes introduced in the new package version. After [the package is automatically built](https://github.com/makedeb/prebuilt-mpr/actions/workflows/update-pkg.yml) and reviewed by a Prebuilt-MPR team member, the package is ready to be updated in the Prebuilt-MPR APT repository.

### 3. The package update
The package has now been built and reviewed by a Prebuilt-MPR team member, and is ready to be updated in the Prebuilt-MPR APT repository. The updated package is [uploaded to the APT repository](https://github.com/makedeb/prebuilt-mpr/actions/workflows/publish-pkg.yml), and is ready to be downloaded for use by end users.

## Reviewing packages
As stated above, all package updates happen in form of pull requests. A Prebuilt-MPR team member needs to review a package's changes before it can be accepted into the Prebuilt-MPR.

Once a package has been built and reviewed, you may merge the package's changes, after which the package will be updated in the Prebuilt-MPR APT repository.

## Adding new packages to the Prebuilt-MPR
All packages in the Prebuilt-MPR are stored alphabetically in [`packages.toml`](/packages.toml). Each package listing takes the following format:

```toml
[pkgname]
blocked_distros = ["focal", "jammy"]
```

`pkgname` is the name of the package (i.e. `docker-compose`).

`blocked_distros` is a list of distributions that the package shouldn't be built on (i.e. it contains incompatible dependencies, etc etc). If there aren't any distributions that the package shouldn't be built on the key may be ommited to only contain the package name like so:

```toml
[pkgname]
```

See `packages.toml` for examples on how the file is layed out.
