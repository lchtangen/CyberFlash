<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';

interface FeedItem {
  title: String;
  link: String;
  description: String;
  pub_date: String;
  source: String;
}

const feeds = ref<FeedItem[]>([]);
const loading = ref(true);
const activeSource = ref('all');

const sources = [
  { id: 'all', label: 'All Feeds' },
  { id: 'xda', label: 'XDA Developers', url: 'https://www.xda-developers.com/feed/' },
  { id: 'android_police', label: 'Android Police', url: 'https://www.androidpolice.com/feed/' },
  { id: 'lineage', label: 'LineageOS', url: 'https://lineageos.org/feed.xml' },
];

const fetchFeeds = async () => {
  loading.value = true;
  feeds.value = [];
  
  try {
    const promises = sources
      .filter(s => s.id !== 'all')
      .map(s => invoke<FeedItem[]>('fetch_rss_feed', { url: s.url, source: s.label }));
      
    const results = await Promise.all(promises);
    feeds.value = results.flat().sort((a, b) => new Date(b.pub_date as string).getTime() - new Date(a.pub_date as string).getTime());
  } catch (e) {
    console.error('Failed to fetch feeds:', e);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  fetchFeeds();
});

const openLink = (url: string) => {
  window.open(url, '_blank');
};

const formatDate = (dateStr: string) => {
  try {
    return new Date(dateStr).toLocaleDateString(undefined, { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
  } catch {
    return dateStr;
  }
};
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-sm font-bold text-white flex items-center gap-2">
        <span class="material-symbols-rounded text-primary">rss_feed</span>
        Community Pulse
      </h3>
      <button @click="fetchFeeds" class="text-white/50 hover:text-white transition-colors">
        <span class="material-symbols-rounded text-sm" :class="{ 'animate-spin': loading }">refresh</span>
      </button>
    </div>

    <div class="flex gap-2 mb-4 overflow-x-auto custom-scrollbar pb-2">
      <button 
        v-for="source in sources" 
        :key="source.id"
        @click="activeSource = source.id"
        class="px-3 py-1 rounded-full text-[10px] font-bold transition-all whitespace-nowrap border"
        :class="activeSource === source.id ? 'bg-primary/20 text-primary border-primary/20' : 'bg-white/5 text-white/50 border-white/5 hover:bg-white/10'"
      >
        {{ source.label }}
      </button>
    </div>

    <div class="flex-1 overflow-y-auto custom-scrollbar space-y-3 pr-2">
      <div v-if="loading && feeds.length === 0" class="text-center py-10 text-white/30 text-xs">
        Loading feeds...
      </div>

      <div 
        v-for="(item, index) in feeds" 
        :key="index"
        v-show="activeSource === 'all' || item.source === sources.find(s => s.id === activeSource)?.label"
        class="p-3 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors cursor-pointer group"
        @click="openLink(item.link as string)"
      >
        <div class="flex justify-between items-start gap-2 mb-1">
          <span class="text-[10px] font-bold text-primary bg-primary/10 px-1.5 py-0.5 rounded">{{ item.source }}</span>
          <span class="text-[10px] text-white/30">{{ formatDate(item.pub_date as string) }}</span>
        </div>
        <h4 class="text-xs font-bold text-white group-hover:text-primary transition-colors line-clamp-2 mb-1">
          {{ item.title }}
        </h4>
        <p class="text-[10px] text-white/50 line-clamp-2" v-html="item.description"></p>
      </div>
    </div>
  </GlassCard>
</template>
