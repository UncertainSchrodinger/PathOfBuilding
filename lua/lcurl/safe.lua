-- Hacky wrapper for curl. I'm not sure how to do native modules in Lua,
-- so I'm just exposing parts in global and then calling those.

local curl = {}
local easy = {}
local new_easy_proxy = nil

-- All values I could find.
-- And not a single timeout in sight.
curl.OPT_ACCEPT_ENCODING = "OPT_ACCEPT_ENCODING"
curl.OPT_IPRESOLVE = "OPT_IPRESOLVE"
curl.OPT_HTTPHEADER = "OPT_HTTPHEADER"
curl.OPT_USERAGENT = "OPT_USERAGENT"
curl.OPT_ACCEPT_ENCODING = "OPT_ACCEPT_ENCODING"
curl.OPT_POST = "OPT_POST"
curl.OPT_POSTFIELDS = "OPT_POSTFIELDS"
curl.OPT_IPRESOLVE = "OPT_IPRESOLVE"
curl.OPT_PROXY = "OPT_PROXY"
curl.INFO_RESPONSE_CODE = "INFO_RESPONSE_CODE"
curl.INFO_SIZE_DOWNLOAD = "INFO_SIZE_DOWNLOAD"
curl.INFO_REDIRECT_URL = "INFO_REDIRECT_URL"
curl.OPT_ACCEPT_ENCODING = "OPT_ACCEPT_ENCODING"
curl.OPT_IPRESOLVE = "OPT_IPRESOLVE"
curl.OPT_PROXY = "OPT_PROXY"
curl.OPT_ACCEPT_ENCODING = "OPT_ACCEPT_ENCODING"
curl.OPT_IPRESOLVE = "OPT_IPRESOLVE"
curl.OPT_PROXY = "OPT_PROXY"
curl.OPT_POST = "OPT_POST"
curl.OPT_USERAGENT = "OPT_USERAGENT"
curl.OPT_POSTFIELDS = "OPT_POSTFIELDS"
curl.OPT_ACCEPT_ENCODING = "OPT_ACCEPT_ENCODING"
curl.OPT_IPRESOLVE = "OPT_IPRESOLVE"
curl.OPT_PROXY = "OPT_PROXY"

curl.setup = function(new_easy)
	new_easy_proxy = new_easy
	easy.new = new_easy_proxy
end

function curl:easy()
	return easy:new()
end

return curl
