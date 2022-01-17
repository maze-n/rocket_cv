# RocketCV

A minimal Rocket server using open-cv rust bindings to perform simple image processing.

## Routes

 - POST http://localhost:8000/blur - Applies a Gaussian blur on the input image:<br>
    `curl --location --request POST 'http://127.0.0.1:8000/blur' \ 
--form 'ksize_height="45"' \
--form 'ksize_width="45"' \
--form 'sigma_x="20"' \
--form 'sigma_y="2"' \
--form 'image=@"/home/maze-n/rocket.jpeg"' --output result.jpeg 
`

 - POST http://localhost:8000/change-size/x/y, where x and y are URL parameters specifying resizing factor in X and Y axes respectively.<br>

    `curl --location --request POST 'http://127.0.0.1:8000/change-size/2/2' \
--form 'image=@"/home/maze-n/rocket.jpeg"' --output result.jpeg`