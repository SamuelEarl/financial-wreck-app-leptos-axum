# IMAGE LOADING & PERFORMANCE NOTES

NOTE: These notes are also in my media-queries.scss file.

* The images for retina displays need to be 2x resolution, which means that the image has twice the width and twice the height of a standard "1x" image, resulting in a total of four times the number of pixels. You can use a free AI image upscaler online to convert standard-resolution images to high-resolution images. For example, Pixelcut.ai has a free image upscaler: https://www.pixelcut.ai/image-upscaler. Pixelcut.ai says that you have 3 downloads per day, but something is wrong with that feature because you are not able to download more than once. This is what you can do to fix that:
    * In Pexelcut.ai, go to the dashboard by clicking the "Create" button. 
    * Click the "Upscale" feature.
    * Upload an image.
    * Select the upscale ratio that you want and click "Upscale".
    * After the image has completed the upscale process, press F12 to open DevTools.
    * Under the "Elements" tab, expand the HTML elements until you find an <img> element. Note that there are 2 <img> elements in the DOM: The original that you uploaded and the scaled version. It looks like the first <img> listed in the DOM is the scaled version.
    * Copy the src URL of the scaled image and paste it in another tab.
    * Right click the image, select "Save image as...", and save your image to your desktop.
* WebP now has broad support in the latest versions of major web browsers. If you don't need to support older browsers, then you can use WebP images throughout your web app instead of providing JPG or PNG fallbacks. (See https://developer.mozilla.org/en-US/docs/Web/Media/Guides/Formats/Image_types#webp_image.)
* Use the appropriate browser cache strategy for your app so that images load quickly. If your content does not change too often (i.e. if you are not a news site, for example), then make sure that your app will cache images in the user's browser. This way if the same image is needed again, the browser doesn't have to go over the network to get it, which is a massive performance boost for repeat viewings. Web browsers cache images by default, but you can change how the browser should cache images. Learn more at https://www.hostwinds.com/blog/what-is-browser-cache.
* Implement lazy loading for images, which is how you can avoid loading all the images on a page at once. Lazy loading means waiting to download an image until it is in or near the viewport. So, for example, an image way far down the page won't load if the user never scrolls there.
