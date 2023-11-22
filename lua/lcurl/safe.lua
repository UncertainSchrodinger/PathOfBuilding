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
end

function curl:easy()
	return easy:new()
end

function easy:new()
	print("eaaasyyyy")
	print(new_easy_proxy)
	local o = {}
	setmetatable(o, self)
	self.__index = self
	return o
end

function easy:setopt_url(url)
	self.url = url
end

function easy:setopt_useragent(url)
	self.url = url
end

function easy:setopt(opt, val)
	print("should set option " .. opt .. " with value " .. val)
end

function easy:setopt_writefunction(writefunction)
	self.writefunction = writefunction
end

function easy:setopt_headerfunction(headerfunction)
	self.headerfunction = headerfunction
end

function easy:perform()
	-- TODO(tatu): trigger call to Rust
	print("should perform http call to url " .. self.url)
end

function easy:getinfo(key)
	-- TODO(tatu): get call details
	print("should get key " .. key)
end

function easy:close()
	-- TODO(tatu): get call details
	print("should close connection")
end

function easy:escape(part)
	-- TODO(tatu): get call details
	print("should escape " .. part)
end

return curl
