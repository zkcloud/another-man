export function formatJson(jsonString) {
  try {
    const parsed = JSON.parse(jsonString);
    return JSON.stringify(parsed, null, 2);
  } catch {
    return jsonString;
  }
}

export function formatXml(xmlString) {
  try {
    const parser = new DOMParser();
    const xmlDoc = parser.parseFromString(xmlString, "text/xml");
    const serializer = new XMLSerializer();
    return serializer.serializeToString(xmlDoc);
  } catch {
    return xmlString;
  }
}

export function detectContentType(contentType, body) {
  if (!contentType) return "text";
  
  const ct = contentType.toLowerCase();
  if (ct.includes("json")) return "json";
  if (ct.includes("xml")) return "xml";
  if (ct.includes("html")) return "html";
  if (ct.includes("javascript")) return "javascript";
  if (ct.includes("css")) return "css";
  
  // Try to detect JSON by content
  try {
    JSON.parse(body);
    return "json";
  } catch {
    return "text";
  }
}

export function formatBody(body, contentType) {
  const type = detectContentType(contentType, body);
  
  switch (type) {
    case "json":
      return formatJson(body);
    case "xml":
    case "html":
      return formatXml(body);
    default:
      return body;
  }
}

export function getLanguageForMonaco(contentType) {
  const type = detectContentType(contentType, "");
  const langMap = {
    json: "json",
    xml: "xml",
    html: "html",
    javascript: "javascript",
    css: "css",
    text: "plaintext",
  };
  return langMap[type] || "plaintext";
}