<script lang="ts">
  import Entry from './components/Entry.svelte';

  export let data: {
    notes: Domain.BlogNote[];
  };

  let title = 'Esteban Borai | Notes';
  let description =
    'Notes taken while reading about computer science and software development.';
  let avatarUrl = 'https://avatars.githubusercontent.com/u/34756077?v=4';
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />
  <!-- Schema.org markup for Google+ -->
  <meta itemprop="name" content={title} />
  <meta itemprop="description" content={description} />
  <meta itemprop="image" content={avatarUrl} />
  <!-- Open Graph data -->
  <meta property="og:title" content={title} />
  <meta property="og:type" content="article" />
  <meta property="og:url" content="https://estebanborai.com/" />
  <meta property="og:image" content={avatarUrl} />
  <meta property="og:description" content={description} />
  <meta property="og:site_name" content="Esteban Borai" />
  <!-- Twitter Card data -->
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:site" content="@EstebanBorai" />
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:creator" content="@EstebanBorai" />
  <meta name="twitter:image:src" content={avatarUrl} />
</svelte:head>

<ul class="flex flex-col md:grid md:grid-cols-3 md:gap-6">
  {#each data.notes as note}
    <Entry
      title={note.title}
      description={note.description}
      publishDate={new Date(note.date)}
      tags={note.categories}
      slug={note.slug}
      previewImageUrl={note.preview_image_url}
    />
  {/each}
</ul>
