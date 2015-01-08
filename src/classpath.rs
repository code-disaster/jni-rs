
use {JNI, Jclass, JmethodID, Jpointer, JNI_NULL};


/// macro to simplify exception checks and return value validation
macro_rules! check_exception_and_result(
    ($jni:expr, $pointer:expr) => {
        match check_exception_and_result($jni, $pointer) {
            true => { println!("classpath exception: Invalid return value at line {}", line!()); break; },
            false => {}
        }
    }
);

pub fn load_static_method(jni:&JNI, classpath_url:&str, class_name:&str) -> (Jclass, JmethodID) {

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

        let url_class = jni.find_class("java/net/URL");
        check_exception_and_result!(jni, url_class);

        let url_ctor = jni.get_method_id(url_class, "<init>", "(Ljava/lang/String;)V");
        check_exception_and_result!(jni, url_ctor);

        let url_str = jni.new_string_utf(classpath_url);
        check_exception_and_result!(jni, url_str);

        let url = jni.new_object_a(url_class, url_ctor, &[url_str]);
        check_exception_and_result!(jni, url);

        // url => [url]
        let url_array = jni.new_object_array(1, url_class, url);
        check_exception_and_result!(jni, url_array);

        // Thread thread = Thread.currentThread();

        let thread_class = jni.find_class("java/lang/Thread");
        check_exception_and_result!(jni, thread_class);

        let thread_get_current = jni.get_static_method_id(thread_class, "currentThread", "()Ljava/lang/Thread;");
        check_exception_and_result!(jni, thread_get_current);

        let thread = jni.call_static_object_method_a(thread_class, thread_get_current, &[]);
        check_exception_and_result!(jni, thread);

        // ClassLoader contextClassLoader = thread.getContextClassLoader();

        let thread_get_loader = jni.get_method_id(thread_class, "getContextClassLoader", "()Ljava/lang/ClassLoader;");
        check_exception_and_result!(jni, thread_get_loader);

        //let loader_class = jni.find_class("java/lang/ClassLoader");

        let loader = jni.call_object_method_a(thread, thread_get_loader, &[]);
        check_exception_and_result!(jni, loader);

        // ClassLoader urlClassLoader = URLClassLoader.newInstance(url, contextClassLoader);

        let url_loader_class = jni.find_class("java/net/URLClassLoader");
        check_exception_and_result!(jni, url_loader_class);

        let url_loader_newinstance = jni.get_static_method_id(url_loader_class, "newInstance", "([Ljava/net/URL;Ljava/lang/ClassLoader;)Ljava/net/URLClassLoader;");
        check_exception_and_result!(jni, url_loader_newinstance);

        let url_loader = jni.call_static_object_method_a(url_loader_class, url_loader_newinstance, &[url_array, loader]);
        check_exception_and_result!(jni, url_loader);

        // thread.setContextClassLoader(urlClassLoader);

        let thread_set_loader = jni.get_method_id(thread_class, "setContextClassLoader", "(Ljava/lang/ClassLoader;)V");
        check_exception_and_result!(jni, thread_set_loader);

        jni.call_void_method_a(thread, thread_set_loader, &[url_loader]);

        // Class<?> mainClass = urlClassLoader.loadClass(<main-class-name>)

        let load_class = jni.get_method_id(url_loader_class, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;");
        check_exception_and_result!(jni, load_class);

        let main_class_name_utf = jni.new_string_utf(class_name);
        check_exception_and_result!(jni, main_class_name_utf);

        let main_class = jni.call_object_method_a(url_loader, load_class, &[main_class_name_utf]);
        check_exception_and_result!(jni, main_class) ;

        // method: 'void main(String[])'

        let main_method = jni.get_static_method_id(main_class, "main", "([Ljava/lang/String;)V");
        check_exception_and_result!(jni, main_method);

        return (main_class, main_method);
    }

    (JNI_NULL, JNI_NULL)
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
