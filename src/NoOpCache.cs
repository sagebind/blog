using System;
using System.Collections.Generic;
using Microsoft.Extensions.Caching.Memory;
using Microsoft.Extensions.Primitives;

namespace Blog
{
    public class NoOpCache : IMemoryCache
    {
        public ICacheEntry CreateEntry(object key)
        {
            return new Entry
            {
                Key = key,
            };
        }

        public void Dispose()
        {
        }

        public void Remove(object key)
        {
        }

        public bool TryGetValue(object key, out object value)
        {
            value = null;
            return false;
        }

        private class Entry : ICacheEntry
        {
            public object Key { get; internal set; }
            public object Value { get; set; }
            public DateTimeOffset? AbsoluteExpiration { get; set; }
            public TimeSpan? AbsoluteExpirationRelativeToNow { get; set; }
            public TimeSpan? SlidingExpiration { get; set; }
            public IList<IChangeToken> ExpirationTokens { get; internal set; }
            public IList<PostEvictionCallbackRegistration> PostEvictionCallbacks { get; internal set; }
            public CacheItemPriority Priority { get; set; }
            public long? Size { get; set; }
            public void Dispose()
            {
            }
        }
    }
}
