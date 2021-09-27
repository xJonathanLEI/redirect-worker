export async function handleRequest(request: Request): Promise<Response> {
  const reqUrl = new URL(request.url);

  if (TARGET_FULL_URL) {
    return new Response(null, {
      status: 301,
      headers: {
        Location: TARGET_FULL_URL,
      },
    });
  } else if (TARGET_BASE_URL) {
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
  } else {
    throw new Error("TARGET_FULL_URL and TARGET_BASE_URL empty");
  }
}
