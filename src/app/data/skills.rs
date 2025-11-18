use std::collections::HashMap;

pub fn skills() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("rust", "devicon-rust-original colored"),
        ("c", "devicon-c-plain colored"),
        ("c++", "devicon-cplusplus-plain colored"),
        ("python", "devicon-python-plain colored"),
        ("javascript", "devicon-javascript-plain colored"),
        ("typescript", "devicon-typescript-plain colored"),
        ("java", "devicon-java-plain colored"),
        ("c#", "devicon-csharp-plain colored"),
        ("go", "devicon-go-original-wordmark colored"),
        ("gitlab", "devicon-gitlab-plain colored"),
        ("github", "devicon-github-original colored"),
        ("git", "devicon-git-plain colored"),
        ("docker", "devicon-docker-plain colored"),
        ("cmake", "devicon-cmake-plain colored"),
        ("react.js", "devicon-react-original colored"),
        ("solid.js", "devicon-solidjs-plain colored"),
        ("next.js", "devicon-nextjs-original-wordmark colored"),
    ])
}
