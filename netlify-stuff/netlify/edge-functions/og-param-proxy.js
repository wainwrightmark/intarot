export default async (request, context) => {
    const url = new URL(request.url)

    // Get the page content.
    const response = await context.next()
    const page = await response.text()

	const id = url.searchParams.get("id");

    if (id == null){
        return response;
    }

    const decoded =  atob(decodeURIComponent());


    const search = 'Logo/full+logo+small.jpg'
    const replace = `Upscaled+Images/${id}.jpg`

    return new Response(page .replaceAll(search, replace), response);
}