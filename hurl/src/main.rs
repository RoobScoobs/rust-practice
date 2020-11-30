/***
 * 
 * 
    COMMAND LINE APPLICATION

    Building a simple command line application for making HTTP requests
    with the following basic feature set:
        - parse the response if it is JSON
        - print out the information about the request including headers
        - print out the response as nicely as possible

    Make a get request to example.com:
        
        hurl example.com

    Make a POST request to example.com with a data JSON object as 
        {
            "foo": "bar"
        }
    
        hurl example.com foo=bar

    Make a PUT request to example.com using HTTPS,
    set the X-API-TOKEN header to abc123,
    and upload a file named foo.txt with the form field name info:

        hurl -s -f PUT example.com X-API-TOKEN:abc123 info@foo.txt

    The -s for --secure option used to turn on HTTPS
    and -f option used to turn on sending as a form

    Headers are to be specified with a colon separating the name from the value

    Files are to be uploaded by using @ between the form field name
    and the path to the file on disk

    Make a form POST request to example.com,
    use bob as the username for basic authentication,
    set the query parameter of foo equal to bar,
    and set the form field option equal to all:

        hurl -f -a bob POST example.com foo==bar option=all

        *before making the request,
        the user should be prompted for their password
        which should be used for the basic authentication scheme*

    
***/