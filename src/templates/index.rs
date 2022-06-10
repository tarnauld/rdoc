pub fn template_index<'index>() -> &'index str {
    "<html>
        <head>
            <title>Rdoc - HTML Report</title>
            <meta name=\"Metadata::Author\" content=\"Rdoc\"/>
            <link rel=\"stylesheet\" href=\"./style.css\"/>
        </head>
        <body>
            {header}
            <div class=\"items-container\">
                {commits}
            </div>
            {footer}
        </body>
    </html>"
}
