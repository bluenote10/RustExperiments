Could not get this to work, because

```
sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libappindicator3-dev
```

failed with

```
The following packages have unmet dependencies:
 libayatana-appindicator3-1 : Conflicts: libappindicator3-1
E: Error, pkgProblemResolver::Resolve generated breaks, this may be caused by held packages.
```

and is required for compilation.
