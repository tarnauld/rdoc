pub fn template_commit<'commit>() -> &'commit str {
    "<html>
        <head>
            <title>Rdoc - HTML Report</title>
            <meta name=\"Metadata::Author\" content=\"Rdoc\"/>
            <link rel=\"stylesheet\" href=\"./style.css\"/>
            <link rel=\"icon\" href=\"./favicon.svg\" type=\"image/svg+xml\">
        </head>
        <body>
            {header}
            <div class=\"commit-container\">
                <div class=\"commit-id\">
                    {fingerprint}
                    <h1>{commit_id}</h1>
                </div>
                <hr/>
                <h3>{commit_date}</h3>
                <div>{commit_authors}</div>
                <p>
                    {commit_message}
                </p>
                <div>
                    {commit_description}
                </div>
            </div>
            {footer}
        </body>
    </html>"
}
