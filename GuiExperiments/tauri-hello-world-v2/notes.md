# Dev flow

First source the `env.sh`.

Then make sure there is some device:

```sh
adb devices
```

Then I had to do the following once:

```sh
npm run tauri android init
```

which creates the `src-tauri/gen` folder which contains the Android (Studio?) project export.

From here, the following started to work:

```sh
npm run tauri android dev
```

Unfortunate the disk space overhead is pretty crazy with ~7 GB for a simple hello world.

An alternative may be to use release builds straight away:

```sh
npm run tauri android build
adb install src-tauri/gen/android/app/build/outputs/apk/arm64/release/app-arm64-release.apk
```

