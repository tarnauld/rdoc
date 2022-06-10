pub fn template_css<'style>() -> &'style str {
    "html, body {
        width: 100%;
        height: 100%;
    }
    
    body {
        background-color: white;
        margin: 0;
    }
    
    .header {
        display: flex;
        align-items: center;
        margin: 0;
        height: 50px;
        width: 100%;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12), 0 1px 2px rgba(0, 0, 0, 0.24);
        transition: all 0.3s cubic-bezier(.25, .8, .25, 1);
    }
    
    .header span {
        font-family: 'Courier New', Courier, monospace;
        font-size: 2em;
        padding-left: 20px;
    }"
}
