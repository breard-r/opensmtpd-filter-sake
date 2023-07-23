const sw_version = '0.1.0';
const cache_name = `sake-v${sw_version}`;
const cached_files = [
	'/app.js',
	'/index.html',
	'/vendor/base32.min.js',
	'/vendor/base32.min.js.map',
	'/vendor/sha256.min.js',
];

function log_message(msg) {
	console.log(`[Service Worker] v${sw_version}: ${msg}`);
}

self.addEventListener('install', (e) => {
	log_message('Install');
	e.waitUntil((async () => {
		const cache = await caches.open(cache_name);
		log_message('Caching all');
		await cache.addAll(cached_files);
	})());
});

self.addEventListener('fetch', (e) => {
	if (!(e.request.url.startsWith('https:') || e.request.url.startsWith('http:'))) {
		log_message(`Fetching resource failed: invalid protocol: ${e.request.url}`);
		return;
	}

	e.respondWith((async () => {
		log_message(`Fetching resource: ${e.request.url}`);
		const cache_promise = await caches.match(e.request);
		if (cache_promise) {
			log_message(`Resource retrieved from cache: ${e.request.url}`);
			return cache_promise;
		}
		const fetch_promise = await fetch(e.request);
		const cache = await caches.open(cache_name);
		log_message(`Caching new resource: ${e.request.url}`);
		cache.put(e.request, fetch_promise.clone());
		return fetch_promise;
	})());
});
