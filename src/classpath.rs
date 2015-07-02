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
    //! initialization of the JVM. For some reason this didn't work for me on *some* systems.
    //!
    //! This method uses JNI voodoo to get the thread context classloader, construct a file
    //! URL, point it to the user JAR, then use the classloader to load the application class'
    //! static main() method.
    //!
    //! References:
    //! http://stackoverflow.com/questions/20328012/c-plugin-jni-java-classpath
    //! http://www.java-gaming.org/index.php/topic,6516.0

    'exit: loop {

        let url_str = new_string_utf(env, classpath_url);
        check_exception_and_result!(env, url_str);

        // URL url = new File("*.jar").toURI().toURL();
        let file_class = find_class(env, "java/io/File");
        check_exception_and_result!(env, file_class);

        let file_ctor = get_method_id(env, file_class, "<init>", "(Ljava/lang/String;)V");
        check_exception_and_result!(env, file_ctor);

        let file = new_object_a(env, file_class, file_ctor, &[url_str]);
        check_exception_and_result!(env, file);

        let to_uri_method = get_method_id(env, file_class, "toURI", "()Ljava/net/URI;");
        check_exception_and_result!(env, to_uri_method);

        let uri = call_object_method_a(env, file, to_uri_method, &[]);
        check_exception_and_result!(env, uri);

        let uri_class = find_class(env, "java/net/URI");
        check_exception_and_result!(env, uri_class);

        let to_url_method = get_method_id(env, uri_class, "toURL", "()Ljava/net/URL;");
        check_exception_and_result!(env, to_url_method);

        let url = call_object_method_a(env, uri, to_url_method, &[]);
        check_exception_and_result!(env, url);

        let url_class = find_class(env, "java/net/URL");
        check_exception_and_result!(env, url_class);

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

        let loader = call_object_method_a(env, thread, thread_get_loader, &[]);
        check_exception_and_result!(env, loader);

        // URLClassLoader urlClassLoader = new URLClassLoader(new URL[]{}).getClass().cast(contextClassLoader);

        let url_array = new_object_array(env, 1, url_class, url);
        check_exception_and_result!(env, url_array);

        let url_class_loader_class = find_class(env, "java/net/URLClassLoader");
        check_exception_and_result!(env, url_class_loader_class);
/*
        let url_class_loader_ctor = get_method_id(env, url_class_loader_class, "<init>", "([Ljava/net/URL;)V");
        check_exception_and_result!(env, url_class_loader_ctor);

        let url_class_loader = new_object_a(env, url_class_loader_class, url_class_loader_ctor, &[url_str]);
        check_exception_and_result!(env, url_class_loader);

        let url_class_loader_get_class = get_method_id(env, url_class_loader_class, "getClass", "()Ljava/lang/Class;");
        check_exception_and_result!(env, url_class_loader);

        let url_class_loader_class_object = call_object_method_a(env, url_class_loader, url_class_loader_get_class, &[]);
        check_exception_and_result!(env, url_class_loader_class_object);

        let class_class = find_class(env, "java/lang/Class");
        check_exception_and_result!(env, class_class);

        let url_class_loader_cast_method = get_method_id(env, class_class, "cast", "(Ljava/lang/Object;)Ljava/lang/Object;");
        check_exception_and_result!(env, url_class_loader_cast_method);

        let url_class_loader_cast = call_object_method_a(env, url_class_loader_class_object, url_class_loader_cast_method, &[loader]);
        check_exception_and_result!(env, url_class_loader_cast);
*/
        let add_url_method = get_method_id(env, url_class_loader_class, "addURL", "(Ljava/net/URL;)V");
        check_exception_and_result!(env, add_url_method);

        call_void_method_a(env, loader/*url_class_loader_cast*/, add_url_method, &[url]);
        check_exception_and_result!(env, 1);

        // Class<?> mainClass = urlClassLoader.loadClass(<main-class-name>)

        let load_class = get_method_id(env, url_class_loader_class, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;");
        check_exception_and_result!(env, load_class);

        let main_class_name_utf = new_string_utf(env, class_name);
        check_exception_and_result!(env, main_class_name_utf);

        let main_class = call_object_method_a(env, loader/*url_class_loader*/, load_class, &[main_class_name_utf]);
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
