export async function handleRequest(request: Request): Promise<Response> {
  const reqUrl = new URL(request.url);

  return new Response(null, {
    status: 301,
    headers: {
      Location:
        (TARGET_BASE_URL.endsWith("/")
          ? TARGET_BASE_URL.substr(0, TARGET_BASE_URL.length - 1)
          : TARGET_BASE_URL) +
        reqUrl.pathname +
        reqUrl.search,
    },
  });
}
