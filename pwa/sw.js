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

self.addEventListener('activate', (e) => {
	e.waitUntil(
		caches.keys().then((keyList) =>
			Promise.all(
				keyList.map((key) => {
					if (key != cache_name) {
						log_message(`Cleaning cache: ${key}`);
						return caches.delete(key);
					}
				}),
			),
		),
	);
	e.waitUntil(clients.claim());
	log_message('Active');
});

self.addEventListener('fetch', (event) => {
	if (!(event.request.url.startsWith('https:') || event.request.url.startsWith('http:'))) {
		log_message(`Fetching resource failed: invalid protocol: ${event.request.url}`);
		return;
	}

	event.respondWith(caches.open(cache_name).then((cache) => {
		log_message(`Fetching resource: ${event.request.url}`);
		return cache.match(event.request).then((cached_response) => {
			const fetched_response = fetch(event.request).then((network_response) => {
				log_message(`Caching resource: ${event.request.url}`);
				cache.put(event.request, network_response.clone());
				return network_response;
			});
			return cached_response || fetched_response;
		});
	}));
});
