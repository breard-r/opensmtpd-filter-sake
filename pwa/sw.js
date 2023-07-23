const sw_version = '0.1.0';
const cache_name = `sake-v${sw_version}`;
const cached_files = [
	'/',
	'/app.js',
	'/index.html',
	'/vendor/base32.min.js',
	'/vendor/base32.min.js.map',
	'/vendor/sha256.min.js',
];

function log_message(msg) {
	console.log(`[Service Worker] v${sw_version}: ${msg}`);
}

self.addEventListener('install', (event) => {
	log_message('Installed');
	self.skipWaiting();
	event.waitUntil(caches.open(cache_name).then((cache) => {
		log_message('Caching all');
		return cache.addAll(cached_files);
	}));
});

self.addEventListener('fetch', (event) => {
	if (!(event.request.url.startsWith('https:') || event.request.url.startsWith('http:'))) {
		log_message(`Fetching resource failed: invalid protocol: ${event.request.url}`);
		return;
	}

	event.respondWith((async () => {
		log_message(`Fetching resource: ${event.request.url}`);
		const cache_promise = await caches.match(event.request);
		if (cache_promise) {
			log_message(`Resource retrieved from cache: ${event.request.url}`);
			return cache_promise;
		}
		const fetch_promise = await fetch(event.request);
		const cache = await caches.open(cache_name);
		log_message(`Caching new resource: ${event.request.url}`);
		cache.put(event.request, fetch_promise.clone());
		return fetch_promise;
	})());
});
