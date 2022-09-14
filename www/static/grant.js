function check(event) {
    const url = "/api/grant";
    var cs124 = document.getElementById('CS124H');
    var cs128 = document.getElementById('CS128H');
    
    var repo_name;
    if (cs128.checked) {
        repo_name = 'pl-cs19628';
    } else {
        repo_name = 'pl-cs124h';
    }

    const data = {
        'netids' : document.getElementById('netids').value,
        'date': document.getElementById('duedate').value,
        'time': document.getElementById('exttime').value,
        'assignment': document.getElementById('assignment').value,
        'repo_name': repo_name,
        'semester': "FA22",
        'github_email': document.getElementById('github_email').value,
        'github_token': document.getElementById('github_token').value,
        'credit': parseInt(document.getElementById('creditfield').value)
    };

    console.log(data);

    fetch("/api/grant", {
        "method": "POST",
        "mode" : "cors",
        "headers": {
            "Content-Type": "application/json; charset=UTF-8"
        },
        "body": JSON.stringify(data)
    })
    .then(function(response) {
        response.json().then((data) => {
            console.log(data);
            document.getElementById('message').innerHTML = data.message;
        });
    })
    .catch(err => {
        console.error(err);
        document.getElementById('message').innerHTML = err.message;
    });
    return false;
}
