pub fn template_commit<'commit>() -> &'commit str {
    "<html>
        <head>
            <title>Rdoc - HTML Report</title>
            <meta name=\"Metadata::Author\" content=\"Rdoc\"/>
            <link rel=\"stylesheet\" href=\"./style.css\"/>
        </head>
        <body>
            {header}
            <div class=\"commit-container\">
                <h1>{commit_id}</h1>
                <div>{commit_authors}</div>
                <h3>{commit_date}</h3>
                <p>
                {commit_message}
                </p>
            </div>
            {footer}
        </body>
    </html>"
}
