pub fn template_css<'style>() -> &'style str {
    "html, body {
        width: 100%;
        height: 100%;
        overflow: auto;
    }
    
    body {
        background-color: white;
        margin: 0;
        display: flex;
        flex-direction: column;
        color: #333;
    }
    
    .header {
        background-color: #02174c;
        display: flex;
        align-items: center;
        margin: 0;
        height: 140px;
        width: 100%;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12), 0 1px 2px rgba(0, 0, 0, 0.24);
        transition: all 0.3s cubic-bezier(.25, .8, .25, 1);
    }
    
    .items-container {
        height: 100%;
        margin: 1.5em;
        display: flex;
        flex-direction: column;
        overflow: auto;
    }
    
    .item {
        background-color: #e6e6e6;
        height: 5em;
        margin: 10px;
        border-radius: 2px;
        box-shadow: 0px 1px 6px #ccc;
        padding: 1em;
        display: flex;
        cursor: pointer;
    }

    .item:hover {
        background-color: #ddd;
    }
    
    .commit-id {
        display: flex;
        flex-direction: column;
        width: 100%;
    }

    a {
        text-decoration: none;
        color: #333;
    }

    hr {
        border-top: 2px solid #bbb;
        border-radius: 2px;
    }

    .title {
        font-size: 2em;
        font-family: Arial, Helvetica, sans-serif;
    }

    .content {
        font-family: Arial, Helvetica, sans-serif;
    }

    .fingerprint {
        fill: #fff;
        background-color: #02174c;
        width: 50px;
        height: 50px;
        border-radius: 50px;
        margin-right: 1em;
    }

    .tags {
        display: flex;
        align-items: center;
    }

    .tags span {
        font-size: 12px;
    }

    .tag {
        width: fit-content;
        margin: 5px;
        padding: 5px;
        background-color: #a5d6a7;
        border-radius: 20px;
        font-family: Arial, Helvetica, sans-serif;
        font-size: 0.7rem;
        color: #555;
    }

    .authors {
        display: flex;
        align-items: center;
    }

    .authors span {
        font-size: 12px;
    }

    .author {
        width: fit-content;
        margin: 5px;
        padding: 5px;
        background-color: #ccc;
        border-radius: 20px;
        font-family: Arial, Helvetica, sans-serif;
        font-size: 0.7rem;
        color: #555;
    }

    .fingerprint-container {
        display: flex;
        align-items: center;
    }

    .commit-container {
        height: 100%;
        background-color: #e6e6e6;
        margin: 1.5em;
        border-radius: 5px;
        box-shadow: 0px 1px 6px #ccc;
        padding: 1.5em;
        font-family: Arial, Helvetica, sans-serif;
    }

    .commit-container .commit-id {
        display: flex;
        flex-direction: row;
    }

    .commit-container .commit-id h1 {
        font-size: 2em;
        font-family: Arial, Helvetica, sans-serif;
        font-weight: normal;
        color: #333;
    }

    .footer {
        content: '';
        width: 100%;
        height: 50px;
        background-color: #02174c;
    }
    
    "
}
