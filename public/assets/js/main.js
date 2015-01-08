window.addEvent('domready', function() {
    SyntaxHighlighter.autoloader(
        ['applescript',                 '/js/shBrushAppleScript.js'],
        ['actionscript3', 'as3',        '/js/shBrushAS3.js'],
        ['bash', 'shell',               '/js/shBrushBash.js'],
        ['coldfusion', 'cf',            '/js/shBrushColdFusion.js'],
        ['cpp', 'c',                    '/js/shBrushCpp.js'],
        ['c#', 'c-sharp', 'csharp',     '/js/shBrushCSharp.js'],
        ['css',                         '/js/shBrushCss.js'],
        ['delphi', 'pascal',            '/js/shBrushDelphi.js'],
        ['diff', 'patch', 'pas',        '/js/shBrushDiff.js'],
        ['erl', 'erlang',               '/js/shBrushErlang.js'],
        ['groovy',                      '/js/shBrushGroovy.js'],
        ['java',                        '/js/shBrushJava.js'],
        ['jfx', 'javafx',               '/js/shBrushJavaFX.js'],
        ['js', 'jscript', 'javascript', '/js/shBrushJScript.js'],
        ['perl', 'pl',                  '/js/shBrushPerl.js'],
        ['php',                         '/js/shBrushPhp.js'],
        ['text', 'plain',               '/js/shBrushPlain.js'],
        ['py', 'python',                '/js/shBrushPython.js'],
        ['ruby', 'rails', 'ror', 'rb',  '/js/shBrushRuby.js'],
        ['sass', 'scss',                '/js/shBrushSass.js'],
        ['scala',                       '/js/shBrushScala.js'],
        ['sql',                         '/js/shBrushSql.js'],
        ['vb', 'vbnet',                 '/js/shBrushVb.js'],
        ['xml', 'xhtml', 'xslt', 'html','/js/shBrushXml.js']
    );
     
    SyntaxHighlighter.all();
});