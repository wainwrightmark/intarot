export default async (request, context) => {
    const url = new URL(request.url)

    // Get the page content.
    const response = await context.next()
    const page = await response.text()

    try{
        const id = url.searchParams.get("id");
        const decoded =  atob(decodeURIComponent(id));
        const search = 'Logo/full+logo+small.jpg'
        const replace = `Upscaled+Images/${decoded}.jpg`

        return new Response(page .replaceAll(search, replace), response);
    }
    catch{
        return response;
    }


}