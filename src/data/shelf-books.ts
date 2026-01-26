export interface ShelfLink {
  name: string;
  url: string;
}

export interface ShelfItem {
  id: string;
  name: string;
  meta: string;
  note: string;
  links: ShelfLink[];
  favorite?: boolean;
  tag?: 'wishlist' | 'blocked';
  tagLabel?: string;
}

export interface ShelfCategory {
  title: string;
  items: ShelfItem[];
  groupWith?: string;
}

export const bookCategories: ShelfCategory[] = [
  {
    title: "Tools for Building",
    groupWith: "group1",
    items: [
      {
        id: "designing-data-intensive-applications",
        name: "Designing Data-Intensive Applications",
        meta: "Martin Kleppmann, 2017",
        note: "The distributed systems bible",
        links: [
          { name: "Official", url: "https://dataintensive.net/" },
          { name: "O'Reilly", url: "https://www.oreilly.com/library/view/designing-data-intensive-applications/9781491903063/" },
        ],
        favorite: true,
      },
      {
        id: "designing-data-intensive-applications-2nd",
        name: "Designing Data-Intensive Applications (2nd Edition)",
        meta: "Martin Kleppmann, 2026",
        note: "The update we've been waiting for",
        links: [
          { name: "Official", url: "https://dataintensive.net/" },
          { name: "Amazon", url: "https://www.amazon.co.uk/dp/1098119061" },
        ],
        tag: "wishlist",
      },
      {
        id: "domain-modeling-made-functional",
        name: "Domain Modeling Made Functional",
        meta: "Scott Wlaschin, 2018",
        note: "Changed how I think about types",
        links: [
          { name: "Official", url: "https://fsharpforfunandprofit.com/ddd/" },
          { name: "Pragmatic", url: "https://pragprog.com/titles/swdddf/domain-modeling-made-functional/" },
        ],
        favorite: true,
      },
      {
        id: "flow-based-programming",
        name: "Flow-Based Programming (2nd Edition)",
        meta: "J. Paul Morrison, 2010",
        note: "Programs as data pipelines",
        links: [
          { name: "Official", url: "https://jpaulm.github.io/fbp/" },
          { name: "Amazon", url: "https://www.amazon.com/Flow-Based-Programming-2nd-Application-Development/dp/1451542321" },
        ],
      },
      {
        id: "lucid-dataflow-programming",
        name: "Lucid, the Dataflow Programming Language",
        meta: "William Wadge & Edward Ashcroft, 1985",
        note: "Time as a first-class concept",
        links: [
          { name: "Wiki", url: "https://en.wikipedia.org/wiki/Lucid_(programming_language)" },
          { name: "PDF", url: "https://worrydream.com/refs/Wadge_1995_-_Lucid,_the_Dataflow_Programming_Language.pdf" },
          { name: "Amazon", url: "https://www.amazon.com/Lucid-Dataflow-Programming-Language-Ashcroft/dp/0127296506" },
        ],
        favorite: true,
      },
    ],
  },
  {
    title: "Design",
    groupWith: "group1",
    items: [
      {
        id: "elm-ui-guide",
        name: "elm-ui: The CSS Escape Plan",
        meta: "Alex Korban, 2019",
        note: "UI without CSS battles",
        links: [
          { name: "Official", url: "https://korban.net/elm/elm-ui-guide/" },
        ],
      },
      {
        id: "refactoring-ui",
        name: "Refactoring UI",
        meta: "Adam Wathan & Steve Schoger, 2018",
        note: "Design taste for developers",
        links: [
          { name: "Official", url: "https://www.refactoringui.com/" },
        ],
        favorite: true,
      },
    ],
  },
  {
    title: "Expanding Horizons",
    groupWith: "group1",
    items: [
      {
        id: "all-i-really-need-to-know",
        name: "All I Really Need to Know I Learned in Kindergarten",
        meta: "Robert Fulghum, 1986",
        note: "Simple truths, easily forgotten",
        links: [
          { name: "Official", url: "https://robertfulghum.com/" },
          { name: "Amazon", url: "https://www.amazon.com/Really-Need-Know-Learned-Kindergarten/dp/034546639X" },
        ],
      },
      {
        id: "empaths-survival-guide",
        name: "The Empath's Survival Guide",
        meta: "Judith Orloff, 2017",
        note: "Feeling everything, on purpose",
        links: [
          { name: "Official", url: "https://drjudithorloff.com/" },
          { name: "Amazon", url: "https://www.amazon.com/Empaths-Survival-Guide-Strategies-Sensitive/dp/1622036573" },
        ],
      },
      {
        id: "future-of-fusion-energy",
        name: "The Future of Fusion Energy",
        meta: "Jason Parisi & Justin Ball, 2019",
        note: "The hardest engineering problem",
        links: [
          { name: "Official", url: "https://www.justin-ball.com/the-future-of-fusion-energy/" },
          { name: "Amazon", url: "https://www.amazon.com/Future-Fusion-Energy-Jason-Parisi/dp/1786345420" },
        ],
      },
      {
        id: "thinking-fast-and-slow",
        name: "Thinking, Fast and Slow",
        meta: "Daniel Kahneman, 2011",
        note: "Why we fool ourselves",
        links: [
          { name: "Amazon", url: "https://www.amazon.com/Thinking-Fast-Slow-Daniel-Kahneman/dp/0374533555" },
        ],
        favorite: true,
      },
      {
        id: "way-of-peaceful-warrior",
        name: "Way of the Peaceful Warrior",
        meta: "Dan Millman, 1980",
        note: "The present moment, practiced",
        links: [
          { name: "Official", url: "https://www.peacefulwarrior.com/" },
          { name: "Amazon", url: "https://www.amazon.com/Way-Peaceful-Warrior-Changes-Lives/dp/1932073205" },
        ],
      },
    ],
  },
  {
    title: "Stories That Stay",
    groupWith: "group2",
    items: [
      {
        id: "100-year-old-man",
        name: "The 100-Year-Old Man Who Climbed Out the Window and Disappeared",
        meta: "Jonas Jonasson, 2009",
        note: "History rewritten with absurdity",
        links: [
          { name: "Official", url: "https://jonasjonasson.com/" },
          { name: "Amazon", url: "https://www.amazon.com/100-Year-Old-Man-Climbed-Window-Disappeared/dp/1611735947" },
        ],
      },
      {
        id: "the-egyptian",
        name: "The Egyptian",
        meta: "Mika Waltari, 1945",
        note: "Ancient world, timeless loneliness",
        links: [
          { name: "Amazon", url: "https://www.amazon.com/Egyptian-Mika-Waltari-ebook/dp/B09L5D39S5" },
        ],
      },
      {
        id: "inheritance-cycle",
        name: "Inheritance Cycle",
        meta: "Christopher Paolini, 2003",
        note: "Dragons and growing up",
        links: [
          { name: "Official", url: "https://www.paolini.net/biographies/christopher-paolini-full/inheritance-cycle/" },
          { name: "Amazon", url: "https://www.amazon.com/Eragon-Inheritance-Book-Christopher-Paolini/dp/0375826696" },
        ],
        favorite: true,
      },
      {
        id: "shadows-of-the-apt",
        name: "Shadows of the Apt",
        meta: "Adrian Tchaikovsky, 2008",
        note: "Insect-kinden and industrial war",
        links: [
          { name: "Official", url: "https://adriantchaikovsky.com/shadows-of-the-apt-series.html" },
          { name: "Amazon", url: "https://www.amazon.com/Empire-Black-Gold-Shadows-Apt/dp/1616141921" },
        ],
      },
      {
        id: "to-sleep-in-a-sea-of-stars",
        name: "To Sleep in a Sea of Stars",
        meta: "Christopher Paolini, 2020",
        note: "First contact done right",
        links: [
          { name: "Official", url: "https://www.paolini.net/works/to-sleep-in-a-sea-of-stars/" },
          { name: "Amazon", url: "https://www.amazon.com/Sleep-Sea-Stars-Christopher-Paolini/dp/1250762847" },
        ],
      },
      {
        id: "the-wandering-earth",
        name: "The Wandering Earth",
        meta: "Liu Cixin, 2000",
        note: "Scale beyond imagination",
        links: [
          { name: "Amazon", url: "https://www.amazon.com/Wandering-Earth-Cixin-Liu/dp/1250796830" },
        ],
      },
      {
        id: "the-witcher",
        name: "The Witcher",
        meta: "Andrzej Sapkowski, 1993",
        note: "Moral gray in every choice",
        links: [
          { name: "Official", url: "https://www.sapkowskibooks.com/" },
          { name: "Amazon", url: "https://www.amazon.com/Last-Wish-Introducing-Witcher/dp/0316029181" },
        ],
      },
    ],
  },
  {
    title: "Ways of Working",
    groupWith: "group2",
    items: [
      {
        id: "rework",
        name: "Rework",
        meta: "Jason Fried & DHH, 2010",
        note: "Permission to stay small",
        links: [
          { name: "Official", url: "https://basecamp.com/books/rework" },
          { name: "Amazon", url: "https://www.amazon.com/Rework-Jason-Fried/dp/0307463745" },
        ],
      },
      {
        id: "shape-up",
        name: "Shape Up",
        meta: "Ryan Singer, 2019",
        note: "How to scope and ship",
        links: [
          { name: "Official", url: "https://basecamp.com/shapeup" },
        ],
        favorite: true,
      },
      {
        id: "working-in-public",
        name: "Working in Public",
        meta: "Nadia Eghbal, 2020",
        note: "The economics of open source",
        links: [
          { name: "Official", url: "https://press.stripe.com/working-in-public" },
          { name: "Amazon", url: "https://www.amazon.com/Working-Public-Making-Maintenance-Software/dp/0578675862" },
        ],
      },
    ],
  },
  {
    title: "For the Little Ones",
    groupWith: "group2",
    items: [
      {
        id: "computer-engineering-for-babies",
        name: "Computer Engineering for Babies",
        meta: "Chase Roberts, 2021",
        note: "Logic gates with colors",
        links: [
          { name: "Official", url: "https://hackylabs.com/en-eu/products/computer-engineering-for-babies" },
        ],
        favorite: true,
      },
      {
        id: "computer-engineering-for-big-babies",
        name: "Computer Engineering for Big Babies",
        meta: "Chase Roberts, 2022",
        note: "Building a CPU, toddler edition",
        links: [
          { name: "Official", url: "https://hackylabs.com/en-eu/products/computer-engineering-for-big-babies" },
        ],
      },
      {
        id: "pepper-and-carrot",
        name: "Pepper & Carrot",
        meta: "David Revoy, 2017",
        note: "Open-source magic",
        links: [
          { name: "Official", url: "https://www.davidrevoy.com/static9/shop" },
        ],
      },
    ],
  },
];
