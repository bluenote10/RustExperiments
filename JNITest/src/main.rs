use jni::*;
use jni::objects::*;
use jni::sys::*;
use jni::JavaVM;

fn call_into_jni() { // -> Result<(), jni::errors::Error> {
    // https://docs.rs/jni/0.13.1/jni/struct.JavaVM.html#launching-jvm-from-rust
    // requires: export LD_LIBRARY_PATH=/home/fabian/bin/jdk1.8.0_74/jre/lib/amd64/server/

    // Build the VM properties
    let jvm_args = InitArgsBuilder::new()
            // Pass the JNI API version (default is 8)
            .version(JNIVersion::V8)
            // You can additionally pass any JVM options (standard, like a system property,
            // or VM-specific).
            // Here we enable some extra JNI checks useful during development
            .option("-Djava.class.path=/home/fabian/git/GuitarGeeksVR/target/scala-2.11/GuitarGeeksVR-assembly-0.1.0.jar")
            .option("-Xcheck:jni")
            .build()
            .unwrap();

    // Create a new VM
    let jvm = JavaVM::new(jvm_args).unwrap();

    // Attach the current thread to call into Java — see extra options in
    // "Attaching Native Threads" section.
    //
    // This method returns the guard that will detach the current thread when dropped,
    // also freeing any local references created in it
    let env = jvm.attach_current_thread().unwrap();

    /*
    let cls = env.find_class("zentabs/PlayerHelloWorld").expect("missing class");
    env.call_static_method(cls, "run", "()V", &[]).expect("call failed");
    */

    let java_str = env.new_string("/home/fabian/Desktop/Machine Head - Blood For Blood.gp5").expect("string creation failed");
    let _cls = env.new_object("zentabs/Player", "(Ljava/lang/String;)V", &[JValue::Object(java_str.into())]).expect("missing class");

    // Call Java Math#abs(-10)
    let x = JValue::from(-10);
    let val: jint = env.call_static_method("java/lang/Math", "abs", "(I)I", &[x]).unwrap().i().unwrap();

    println!("{}", val);
    assert_eq!(val, 10);

    std::thread::sleep(std::time::Duration::from_secs(60));
}


fn main() {
    //std::env::set_var("LD_LIBRARY_PATH", "/home/fabian/bin/jdk1.8.0_74/jre/lib/amd64/server/");
    //println!("LD_LIBRARY_PATH = {}", std::env::var("LD_LIBRARY_PATH").unwrap());

    call_into_jni();//.expect("works");
}
