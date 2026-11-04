import { executeGraphQL } from '@/graphql/client';
import { GraphQlFoundationStatusDocument } from '@/graphql/generated/graphql';

/**
 * Minimal typed GraphQL call kept by EVO-127-A as the transport/codegen smoke path.
 * Product data continues to use TanStack Query as the cache layer in later vertical slices.
 */
export function fetchGraphQLFoundationStatus() {
  return executeGraphQL({
    document: GraphQlFoundationStatusDocument,
    variables: {},
  });
}
