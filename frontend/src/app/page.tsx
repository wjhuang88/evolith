import Link from 'next/link';
import { Button } from '@/components/ui/Button';

export default function HomePage() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-background to-background/80">
      <div className="container flex flex-col items-center justify-center gap-8 px-4 py-16">
        {/* Hero Section */}
        <div className="text-center">
          <h1 className="text-5xl font-bold tracking-tight text-foreground sm:text-6xl">
            Evolith
          </h1>
          <p className="mt-4 text-xl text-muted-foreground">
            智能体开发服务平台
          </p>
          <p className="mt-2 text-lg text-muted-foreground/80">
            AI Agent Development Platform
          </p>
        </div>

        {/* Features Grid */}
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          <FeatureCard
            title="MCP Tools"
            description="标准化的工具封装，通过MCP协议提供服务"
            href="/tools"
          />
          <FeatureCard
            title="Skills"
            description="兼容Claude Skills格式的混合型技能系统"
            href="/skills"
          />
          <FeatureCard
            title="Snippets"
            description="面向大模型的代码片段仓库，降低token消耗"
            href="/snippets"
          />
        </div>

        {/* CTA Section */}
        <div className="flex gap-4">
          <Link href="/tools">
            <Button variant="primary" size="lg">
              开始使用
            </Button>
          </Link>
          <Link href="/docs">
            <Button variant="outline" size="lg">
              查看文档
            </Button>
          </Link>
        </div>
      </div>
    </main>
  );
}

function FeatureCard({
  title,
  description,
  href,
}: {
  title: string;
  description: string;
  href: string;
}) {
  return (
    <Link
      href={href}
      className="group rounded-lg border border-border bg-card p-6 transition-colors hover:border-primary hover:bg-card/80"
    >
      <h2 className="mb-2 text-xl font-semibold">{title}</h2>
      <p className="text-sm text-muted-foreground">{description}</p>
    </Link>
  );
}
