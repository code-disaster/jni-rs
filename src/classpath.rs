
use {JNI, Jclass, JmethodID, Jpointer};

macro_rules! check_exception_and_result(
    ($jni:expr, $pointer:expr) => {
        match check_exception_and_result($jni, $pointer) {
            true => { println!("Invalid return value at line {}", line!()); true }
            false => false
        }
    }
)

/// reference: http://stackoverflow.com/questions/20328012/c-plugin-jni-java-classpath
pub fn load_static_method(jni:&JNI, classpath_url:&str, class_name:&str) -> (Jclass, JmethodID) {

    'exit: loop {

        // URL url = new java.net.URL("file://<path-to-jar>");

        let url_class = jni.find_class("java/net/URL");
        if check_exception_and_result!(jni, url_class) {
            break 'exit;
        }

        let url_ctor = jni.get_method_id(url_class, "<init>", "(Ljava/lang/String;)V");
        if check_exception_and_result!(jni, url_ctor) {
            break 'exit;
        }

        let url_str = jni.new_string_utf(classpath_url);
        if check_exception_and_result!(jni, url_str) {
            break 'exit;
        }

        let url = jni.new_object_a(url_class, url_ctor, [url_str]);
        if check_exception_and_result!(jni, url) {
            break 'exit;
        }

        // url => [url]
        let url_array = jni.new_object_array(1, url_class, url);
        if check_exception_and_result!(jni, url_array) {
            break 'exit;
        }

        // Thread thread = Thread.currentThread();

        let thread_class = jni.find_class("java/lang/Thread");
        if check_exception_and_result!(jni, thread_class) {
            break 'exit;
        }

        let thread_get_current = jni.get_static_method_id(thread_class, "currentThread", "()Ljava/lang/Thread;");
        if check_exception_and_result!(jni, thread_get_current) {
            break 'exit;
        }

        let thread = jni.call_static_object_method_a(thread_class, thread_get_current, []);
        if check_exception_and_result!(jni, thread) {
            break 'exit;
        }

        // ClassLoader contextClassLoader = thread.getContextClassLoader();

        let thread_get_loader = jni.get_method_id(thread_class, "getContextClassLoader", "()Ljava/lang/ClassLoader;");
        if check_exception_and_result!(jni, thread_get_loader) {
            break 'exit;
        }

        //let loader_class = jni.find_class("java/lang/ClassLoader");

        let loader = jni.call_object_method_a(thread, thread_get_loader, []);
        if check_exception_and_result!(jni, loader) {
            break 'exit;
        }

        // ClassLoader urlClassLoader = URLClassLoader.newInstance(url, contextClassLoader);

        let url_loader_class = jni.find_class("java/net/URLClassLoader");
        if check_exception_and_result!(jni, url_loader_class) {
            break 'exit;
        }

        let url_loader_newinstance = jni.get_static_method_id(url_loader_class, "newInstance", "([Ljava/net/URL;Ljava/lang/ClassLoader;)Ljava/net/URLClassLoader;");
        if check_exception_and_result!(jni, url_loader_newinstance) {
            break 'exit;
        }

        let url_loader = jni.call_static_object_method_a(url_loader_class, url_loader_newinstance, [url_array, loader]);
        if check_exception_and_result!(jni, url_loader) {
            break 'exit;
        }

        // thread.setContextClassLoader(urlClassLoader);

        let thread_set_loader = jni.get_method_id(thread_class, "setContextClassLoader", "(Ljava/lang/ClassLoader;)V");
        if check_exception_and_result!(jni, thread_set_loader) {
            break 'exit;
        }

        jni.call_void_method_a(thread, thread_set_loader, [url_loader]);
        //check_for_exceptions(jni);

        // Class<?> mainClass = urlClassLoader.loadClass(<main-class-name>)

        let load_class = jni.get_method_id(url_loader_class, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;");
        if check_exception_and_result!(jni, load_class) {
            break 'exit;
        }

        let main_class_name_utf = jni.new_string_utf(class_name);
        if check_exception_and_result!(jni, main_class_name_utf) {
            break 'exit;
        }

        let main_class = jni.call_object_method_a(url_loader, load_class, [main_class_name_utf]);
        if check_exception_and_result!(jni, main_class) {
            break 'exit;
        }

        // method: 'void main(String[])'

        let main_method = jni.get_static_method_id(main_class, "main", "([Ljava/lang/String;)V");
        if check_exception_and_result!(jni, main_method) {
            break 'exit;
        }

        return (main_class, main_method);
    }

    (0u64, 0u64)
}

fn check_exception_and_result(jni:&JNI, pointer:Jpointer) -> bool {
    let throwable = jni.exception_occured();
    if !JNI::is_null(throwable) {
        jni.exception_describe();
        jni.exception_clear();
        return true;
    }

    JNI::is_null(pointer)
}
