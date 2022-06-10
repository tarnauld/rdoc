pub fn template_index<'index>() -> &'index str {
    "<html>
        <head>
            <title>Rdoc - HTML Report</title>
            <meta name=\"Metadata::Author\" content=\"Rdoc\"/>
            <link rel=\"stylesheet\" href=\"./style.css\"/>
        </head>
        <body>
            <div class=\"header\">
                <span>Rdoc HTML Report</span>
            </div>
            <ul>
                {commits}
            </ul>
        </body>
    </html>"
}
