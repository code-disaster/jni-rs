use consts::JNI_NULL;
use ffi::*;
use types::*;

/// macro to simplify exception checks and return value validation
macro_rules! check_exception_and_result(
    ($jni:expr, $pointer:expr) => {
        match check_exception_and_result($jni, $pointer) {
            true => { println!("classpath exception: Invalid return value at line {}", line!()); break; },
            false => {}
        }
    }
);

pub fn load_static_method(env: &mut JNIEnv, classpath_url: &str, class_name: &str) -> (Jclass, JmethodID) {

    //! Method to retrieve 'static void main(String[] args)' from a user-defined class path.
    //! The original 'packr' passes "-Djava.class.path=<path-to-jar>" as an argument during
    //! initialization of the JVM. For some reason this didn't work for me.
    //!
    //! This method uses JNI on java.lang|net.* classes (which are accessible thanks to the
    //! bootstrap classpath) to instantiate a custom URLClassLoader, point it to the user JAR,
    //! then use this classloader to access the correct class' main() method.
    //!
    //! Reference: http://stackoverflow.com/questions/20328012/c-plugin-jni-java-classpath

    'exit: loop {

        // URL url = new java.net.URL("file://<path-to-jar>");

        let url_class = find_class(env, "java/net/URL");
        check_exception_and_result!(env, url_class);

        let url_ctor = get_method_id(env, url_class, "<init>", "(Ljava/lang/String;)V");
        check_exception_and_result!(env, url_ctor);

        let url_str = new_string_utf(env, classpath_url);
        check_exception_and_result!(env, url_str);

        let url = new_object_a(env, url_class, url_ctor, &[url_str]);
        check_exception_and_result!(env, url);

        // url => [url]
        let url_array = new_object_array(env, 1, url_class, url);
        check_exception_and_result!(env, url_array);

        // Thread thread = Thread.currentThread();

        let thread_class = find_class(env, "java/lang/Thread");
        check_exception_and_result!(env, thread_class);

        let thread_get_current = get_static_method_id(env, thread_class, "currentThread", "()Ljava/lang/Thread;");
        check_exception_and_result!(env, thread_get_current);

        let thread = call_static_object_method_a(env, thread_class, thread_get_current, &[]);
        check_exception_and_result!(env, thread);

        // ClassLoader contextClassLoader = thread.getContextClassLoader();

        let thread_get_loader = get_method_id(env, thread_class, "getContextClassLoader", "()Ljava/lang/ClassLoader;");
        check_exception_and_result!(env, thread_get_loader);

        //let loader_class = jni.find_class("java/lang/ClassLoader");

        let loader = call_object_method_a(env, thread, thread_get_loader, &[]);
        check_exception_and_result!(env, loader);

        // ClassLoader urlClassLoader = URLClassLoader.newInstance(url, contextClassLoader);

        let url_loader_class = find_class(env, "java/net/URLClassLoader");
        check_exception_and_result!(env, url_loader_class);

        let url_loader_newinstance = get_static_method_id(env, url_loader_class, "newInstance", "([Ljava/net/URL;Ljava/lang/ClassLoader;)Ljava/net/URLClassLoader;");
        check_exception_and_result!(env, url_loader_newinstance);

        let url_loader = call_static_object_method_a(env, url_loader_class, url_loader_newinstance, &[url_array, loader]);
        check_exception_and_result!(env, url_loader);

        // thread.setContextClassLoader(urlClassLoader);

        let thread_set_loader = get_method_id(env, thread_class, "setContextClassLoader", "(Ljava/lang/ClassLoader;)V");
        check_exception_and_result!(env, thread_set_loader);

        call_void_method_a(env, thread, thread_set_loader, &[url_loader]);

        // Class<?> mainClass = urlClassLoader.loadClass(<main-class-name>)

        let load_class = get_method_id(env, url_loader_class, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;");
        check_exception_and_result!(env, load_class);

        let main_class_name_utf = new_string_utf(env, class_name);
        check_exception_and_result!(env, main_class_name_utf);

        let main_class = call_object_method_a(env, url_loader, load_class, &[main_class_name_utf]);
        check_exception_and_result!(env, main_class) ;

        // method: 'void main(String[])'

        let main_method = get_static_method_id(env, main_class, "main", "([Ljava/lang/String;)V");
        check_exception_and_result!(env, main_method);

        return (main_class, main_method);
    }

    (JNI_NULL, JNI_NULL)
}

fn check_exception_and_result(env: &mut JNIEnv, pointer: Jpointer) -> bool {
    let throwable = exception_occured(env);
    if !is_null(throwable) {
        exception_describe(env);
        exception_clear(env);
        return true;
    }

    is_null(pointer)
}
